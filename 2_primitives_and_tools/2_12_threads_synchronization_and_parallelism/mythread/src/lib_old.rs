

/*
pub struct ThreadPool;

impl ThreadPool {
   /// Create a new ThreadPool.
   ///
   /// The size is the number of threads in the pool.
   ///
   /// # Panics
   ///
   /// The `new` function will panic if the size is zero.
   pub fn new(size: usize) -> ThreadPool {
       assert!(size > 0);

       ThreadPool
   }
   pub fn execute<F>(&self, f: F)
       where
           F: FnOnce() + Send + 'static
   {

   }
}
*/




/*
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            //  будет запускать некоторый код для создания потоков
        }

        ThreadPool {
            threads
        }
    }
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
*/



// Worker Struct отвечает за отправку кода из ThreadPool потока
//  Здесь мы рассмотрим, как мы на самом деле создаем потоки.
// Стандартная библиотека обеспечивает thread::spawn способ создания потоков и thread::spawn ожидает получения некоторого кода, который поток должен запускать, как только создается поток.
// Однако в нашем случае мы хотим создать потоки и заставить их ждать кода, который мы отправим позже.
// Реализация потоков в стандартной библиотеке не включает никаких способов сделать это; мы должны реализовать его вручную.
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;



pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

// Эта черта имеет один метод call_box, который аналогичен call методам других Fn* признаков, за исключением того,
// что требуется self: Box<dyn Self> взять на себя ответственность self и вывести значение из Box<T>.
// Rust еще не понимает, что он может использовать self: Box<dyn Self> в этой ситуации, чтобы взять на себя ответственность за закрытие и оттолкнуть закрытие Box<T>
// но часть Rust, которая реализует поведение при вызове замыкания, не реализуется с использованием self: Box<dyn Self>
// Но пока, давайте обойдем эту проблему, используя удобный трюк. Мы можем прямо сказать Rust,
// что в этом случае мы можем взять на себя ответственность за ценность внутри Box<T> использования self: Box<dyn Self>
trait FnBox {
    fn call_box(self: Box</*dyn*/ Self>);
}
//Эффективно это означает, что любые FnOnce()замыкания могут использовать наш call_box метод.
// Реализация call_box использования (*self)() для перемещения закрытия Box<T> и вызова закрытия.
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box</*dyn */F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;


// Чтобы вызвать FnOnce закрытие, которое хранится в Box<T>(что и является нашим Job псевдонимом типа), закрытие должно выйти из него, Box<T> потому что закрытие становится собственностью, self когда мы его вызываем.
// В общем, Rust не позволяет нам переносить ценность из-за того, Box<T> что Rust не знает, насколько велика ценность внутри Box<T>
// type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }

}
// Внешний код (например, наш сервер ) не обязательно должен знать детали реализации, связанные с использованием Worker структуры внутри ThreadPool,
// поэтому мы делаем Worker структуру и ее new функцию закрытыми.
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}






/*
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                (*job)();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
*/


/*
с while медленный запрос все равно заставит другие запросы ждать обработки.
Причина несколько тонкая: Mutex структура не имеет никакого публичного unlock метода , так как право собственность на замке основана на время жизни в MutexGuard<T> пределах ,
LockResult<MutexGuard<T>> что lock метод возвращает.
Во время компиляции контролер заимствования может затем применить правило, чтобы ресурс, защищенный с помощью, Mutex не мог быть доступен, если мы не удерживаем блокировку.
Но эта реализация также может привести к тому, что блокировка будет удерживаться дольше, чем предполагалось, если мы не будем думать внимательно о времени жизни MutexGuard<T>.
Поскольку значения в whileвыражении остаются в области действия для продолжительности блока, блокировка сохраняется в течение всего времени вызоваjob.call_box(),
что означает, что другие работники не могут получать работу.
*/
// Используя loop вместо while и приобретая блокировку и задание в блоке, а не за его пределами, MutexGuard возвращаемый lock метод отбрасывается, как только let job оператор заканчивается.
// Это гарантирует, что блокировка будет сохранена во время вызова recv, но она будет выпущена до вызова job.call_box(), позволяя одновременно обслуживать несколько запросов.
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
// Успех! Теперь у нас есть пул потоков, который выполняет соединения асинхронно.

