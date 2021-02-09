struct IO<'a, A: 'a> {
    thunk: Box<dyn FnOnce() -> A + 'a>,
}

// somehow it works
// todo: figure out what does it all mean
impl<'b, 'a: 'b, A> IO<'a, A> {
    fn run(self) -> A {
        (self.thunk)()
    }

    fn pure(thunk: A) -> Self {
        IO {
            thunk: Box::new(move || thunk),
        }
    }

    fn map<B: 'b>(self, f: impl FnOnce(A) -> B + 'b) -> IO<'b, B> {
        IO::<'b, B> {
            thunk: Box::new(move || f(self.run())),
        }
    }

    fn flat_map<B: 'b>(self, f: impl FnOnce(A) -> IO<'b, B> + 'b) -> IO<'b, B> {
        IO::<'b, B> {
            thunk: Box::new(move || f(self.run()).run()),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let io = IO::pure(10)
        .map(|x| {
            println!("test {}", x);
            x
        })
        .map(|x| x * 2)
        .map(|x| {
            println!("x {}", x);
            x
        });

    let io2 = IO::pure(20);

    // todo: this needs some macros
    let res = io.flat_map(|x| io2.map(move |q| q * x));

    let xxx = res.run();
    println!("xxx {}", xxx);
}
