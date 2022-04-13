use std::collections::{HashMap, HashSet,BTreeMap};
use std::mem;
use std::sync::{mpsc,Arc, Mutex, MutexGuard};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};
// Что делает синтаксис async / await
// С обычными фьючерсами вы должны написать функции обратного вызова для обработки результатов, когда они станут доступны.
// С помощью async / await вы можете написать «синхронно выглядящий» код, который выглядит так, как будто он просто ожидает значения,
// но фактически преобразуется под капотом в тот же стиль обратного вызова, который вы должны были написать ранее.
// В конце концов, это синтаксический сахар, но он может существенно повлиять на читаемость и размер сложного асинхронного кода.

// poll-based это означает, что после создания будущее не будет выполняться автоматически на месте, но должно быть явно выполнено каким-то исполнителем



// trait ToyTask           - Типаж задачи
// enum Async<T>           - Тип ответа задачи
// trait Wake: Send + Sync - Типаж Пробудитель задачи в поток
// struct ExecState        - Состояние задачи
// struct ToyExec          - Исполнитель
// struct TaskEntry        - Задача

pub mod async;
pub mod exec;
pub mod toy;
pub mod wake;

use exec::ExecState;
use wake::{Waker,ToyWake};
use toy::ToyTask;

/// Задача
pub struct TaskEntry {
    pub task: Box<ToyTask + Send>, // Типаж простой задачи
    pub wake: Waker,               // Waker для пробуждения ее
}

/// Исполнитель
// Что бы позволить использовать состояние из других потоков оборачиваим их Arc<Mutex<ExecState>>
#[derive(Clone)]
pub struct ToyExec {
    pub state: Arc<Mutex<ExecState>>,
}

// несколько шаблонов для создания и работы с исполнителем
impl ToyExec {
    pub fn new() -> Self {
        ToyExec {
            state: Arc::new(Mutex::new(ExecState {
                next_id: 0,
                tasks: HashMap::new(),
                ready: HashSet::new(),
                thread: thread::current(),
            })),
        }
    }

    // метод для удобства получения информации о состоянии исполнителя
    fn state_mut(&self) -> MutexGuard<ExecState> {
        self.state.lock().unwrap()
    }

    /// Основной цикл задачи в исполнителе
    /// Для простоты никогда не выходит, он просто постоянно запускает все нерешенные задачи до завершения
    pub fn run(&self) {
        loop {
            //Каждый раз мы собираем полный набор готовых к выполнению идентификаторов задач:
            let mut ready = mem::replace(&mut self.state_mut().ready, HashSet::new());
            //  mem::replace(dest: &mut T, src: T) -> T  Перемещается src в ссылку dest, возвращая предыдущее dest значение.
            // replace позволяет потреблять поле структуры, заменяя его другим значением.

            // Теперь попробуйте «выполнить» каждую изначально готовых задач:
            for id in ready.drain() {
                // drain() - Очищает набор, возвращая все элементы в итераторе
                // Мы берем  полное право собственности на эту задачу; если он будет завершен, он будет опущен.
                let entry = self.state_mut().tasks.remove(&id);
                if let Some(mut entry) = entry {
                    if let Async::Pending = entry.task.poll(&entry.wake) {
                        // Задача не завершена, поэтому верните ее в таблицу.
                        self.state_mut().tasks.insert(id, entry);
                    }
                }
            }

            // Мы обработали всю работу, которую мы приобрели при входе; блокировать до тех пор, пока не будет доступна дополнительная работа
            // Если новая работа стала доступна после моментального снимка `ready`, это будет no-op.
            thread::park(); // Блокирует, если или пока токен текущего потока не будет доступен.
        }
    }

    // Остальные части являются простыми. spawn Метод отвечает за пакаджа задачу в TaskEntry и установить его:
    // И с этим мы создали планировщик задач!
    fn spawn<T>(&self, task: T) where T: ToyTask + Send + 'static,
    {
        // Заполняем ExecState

        let mut state = self.state_mut();// достаем ExecState

        let id = state.next_id;
        state.next_id += 1;// устанавливаем id задачи

        let wake = ToyWake {
            id,
            exec: self.clone(),
        };
        let entry = TaskEntry {
            wake: Waker::from(Arc::new(wake)),
            task: Box::new(task),
        };
        state.tasks.insert(id, entry);

        // Недавно добавленная задача считается сразу готовой к запуску,
        // которая вызовет последующий вызов `park`, чтобы сразу
        // вернуть.
        state.wake_task(id);// Пробуждение задачи
    }
}

// Давайте перейдем к созданию источника событий для задач, которые ждут.


/// Тип для запроса пробуждения
pub struct Registration {
    pub at: Instant,
    pub wake: Waker,// тип пробудитель
}

/// State for the worker thread that processes timer events
pub struct Worker {
    pub rx: mpsc::Receiver<Registration>,
    pub active: BTreeMap<Instant, Waker>,
}


/// A handle to a timer, used for registering wakeups
#[derive(Clone)]
pub struct ToyTimer {
    pub tx: mpsc::Sender<Registration>,
}

impl ToyTimer {
    fn new() -> ToyTimer {
        // создаем связанные каналом два объекта, отправителя и получателя
        let (tx, rx) = mpsc::channel();// создание канала
        //создание получателя(любого)
        let worker = Worker {
            rx,
            active: BTreeMap::new(),
        };
        thread::spawn(|| worker.work());//получателя запускаем в отдельном потоке
        ToyTimer { tx }// отдаем отправителя
    }

    // Register a new wakeup with this timer
    fn register(&self, at: Instant, wake: Waker) {
        self.tx.send(Registration { at, wake }).unwrap();
    }
}

impl Worker {
    fn enroll(&mut self, item: Registration) {
        if self.active.insert(item.at, item.wake).is_some() {
            // this simple setup doesn't support multiple registrations for
            // the same instant; we'll revisit that in the next section.
            panic!("Attempted to add to registrations for the same instant")
        }
    }

    fn fire(&mut self, key: Instant) {
        self.active.remove(&key).unwrap().wake();
    }

    fn work(mut self) {
        loop {
            if let Some(first) = self.active.keys().next().cloned() {
                let now = Instant::now();
                if first <= now {
                    self.fire(first);
                } else {
                    // we're not ready to fire off `first` yet, so wait until we are
                    // (or until we get a new registration, which might be for an
                    // earlier time).
                    if let Ok(new_registration) = self.rx.recv_timeout(first - now) {
                        self.enroll(new_registration);
                    }
                }
            } else {
                // no existing registrations, so unconditionally block until
                // we receive one.
                let new_registration = self.rx.recv().unwrap();
                self.enroll(new_registration)
            }
        }
    }
}

fn main() {
    let timer = ToyTimer::new();// связали два обьекта Worker и ToyTimer каналом для обмена типом Registration
    let exec = ToyExec::new();// создали исполнителя содержащего объект состояние ExecState с пустым hashmap задач TaskEntry
                              // который содержит пробудителя Worker и любой обьект ToyTask (Periodic)

    for i in 1..10 {
        // отдаем задачу Periodic содержащую объект отправитель ToyTimer
        exec.spawn(Periodic::new(
            i,
            Duration::from_millis(i * 500),
            timer.clone(),
        ));
    }

    exec.run()
}
