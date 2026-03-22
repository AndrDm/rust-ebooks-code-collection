use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;

struct InteraktiveFuture {
    status: InteraktivStatus,
    eingabe: Option<String>,
}

impl InteraktiveFuture {
    pub fn new() -> InteraktiveFuture {
        InteraktiveFuture {
            status: InteraktivStatus::Menue,
            eingabe: None,
        }
    }
}

enum InteraktivStatus {
    Menue,
    MenueAusgewaehlt(String),
    Eingabe,
    Ausgabe(String),
}

impl Future for InteraktiveFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker().clone();

        match &mut self.status {
            InteraktivStatus::Menue => {
                println!("Menü, fortfahren mit:\n---------------------");
                println!("1:\t Eingabe");
                println!("2:\t Beenden\n");

                let mut auswahl = String::new();
                let _ = std::io::stdin().read_line(&mut auswahl);
                self.status = InteraktivStatus::MenueAusgewaehlt(auswahl);
                waker.wake();
            }
            InteraktivStatus::MenueAusgewaehlt(auswahl) => {
                if auswahl.contains("2") {
                    return Poll::Ready(());
                }

                self.status = InteraktivStatus::Eingabe;
                waker.wake();
            }
            InteraktivStatus::Eingabe => {
                use std::io::Write;

                print!("Jetzt eingeben: ");
                std::io::stdout().flush().unwrap();
                let mut auswahl = String::new();
                let _ = std::io::stdin().read_line(&mut auswahl);
                self.status = InteraktivStatus::Ausgabe(auswahl);
                waker.wake();
            }
            InteraktivStatus::Ausgabe(ausgabe) => {
                println!("\nEingelesen: {ausgabe}\n");
                self.status = InteraktivStatus::Menue;
                waker.wake();
            }
        }

        return Poll::Pending;
    }
}

struct InteraktiverWaker;
impl Wake for InteraktiverWaker {
    fn wake(self: Arc<Self>) {
        println!("Aufgeweckt!");
    }
}

// pub fn main() {
//     {
//         // Erster Versuch
//         let mut interaktiv = InteraktiveFuture::new();
//         // Woher kommen die Argumente?
//         // let ergebnis = interaktiv.poll(??);
//     }
//
//     {
//         // Zweiter Versuch
//         let mut interaktiv = InteraktiveFuture::new();
//         let mut pinned = Pin::new(&mut interaktiv);
//
//         // Noch fehlt der Context
//         // let ergebnis = interaktiv.poll(??);
//     }
//
//     {
//         // Dritter Versuch
//         let mut interaktiv = InteraktiveFuture::new();
//         let mut pinned = Pin::new(&mut interaktiv);
//         let waker = Waker::from(Arc::new(InteraktiverWaker));
//         let mut context = Context::from_waker(&waker);
//
//         // let ergebnis = pinned.poll(&mut context);
//     }
//
//     {
//         // Final
//         let mut interaktiv = InteraktiveFuture::new();
//         let mut pinned = Pin::new(&mut interaktiv);
//         let waker = Waker::from(Arc::new(InteraktiverWaker));
//         let mut context = Context::from_waker(&waker);
//
//         // Polling
//         // while let Poll::Pending = pinned.as_mut().poll(&mut context) {
//         //     // ...
//         // };
//     }
//
//     {
//         // Mit Waker aus Package "futures"
//         let mut interaktiv = InteraktiveFuture::new();
//         let mut pinned = Pin::new(&mut interaktiv);
//         let waker = futures::task::noop_waker();
//         let mut context = Context::from_waker(&waker);
//         // let ergebnis = pinned.poll(&mut context);
//     }
// }

fn sync_und_async() -> impl Future {
    println!("Hier synchron");
    async {
        use tokio::time;
        use std::time::Duration;

        time::sleep(Duration::from_secs(1)).await;
        println!("... und hier asynchron!");
    }
}

#[tokio::main]
pub async fn main() {
    sync_und_async().await;

    let mut interaktiv = InteraktiveFuture::new();
    interaktiv.await;
}
