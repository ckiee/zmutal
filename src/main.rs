use std::{
    f64::consts::TAU,
    io::{stdout, Write},
    time::{Duration, Instant}, ops::Mul,
};

enum Wave {
    Amp,
    Phase,
    Freq,
    Sine,
    Saw,
    Quantize,
    Hear,
    Push(f64),
    Mult,
}

fn main() {
    let rate = 44100.0;
    let wavelength = Duration::from_secs_f64(1.0 / rate);
    let mut sample = 0;
    loop {
        let start = Instant::now();
        //
        let fsamp = sample as f64;
        let channels = vec![
            vec![Wave::Push(1.0), Wave::Freq, Wave::Sine],
            vec![Wave::Push(440.0), Wave::Freq, Wave::Push(40.0), Wave::Mult, Wave::Sine, Wave::Quantize, Wave::Hear],
        ];
        let mut out: f64 = 0.0;

        let mut stack = vec![];

        for (idx, ch) in channels.iter().enumerate() {
            let mut phase = 0.0;
            let mut freq = 1.0;
            let chlf = channels.len() as f64;
            let mut amp = 1.0 / (channels.len() as f64);
            let pop = |s: &mut Vec<f64>| s.pop().unwrap();
            for w in ch {
                match *w {
                    Wave::Freq => { freq = pop(&mut stack); },
                    Wave::Amp => { amp = pop(&mut stack) / chlf; },
                    // Wave::Phase(x) => { phase = x; },
                    Wave::Sine => {
                        stack.push((TAU * (fsamp / (rate / freq))).sin() * amp);
                    }
                    Wave::Saw => {
                        stack.push(((fsamp / (rate / freq)).fract() - 0.5) * 2.0 / amp);
                    }
                    Wave::Quantize => {
                        let wave = pop(&mut stack);
                        let by = pop(&mut stack);
                        // Quantize < 1.0 the same as == 1.0 for now. TODO
                        stack.push(if by <= 1.0 {
                            if wave > 0.0 { amp } else { -amp }
                        } else {
                            (wave * by).trunc() / by
                        })
                    },
                    Wave::Hear => {
                        out += pop(&mut stack);
                    },
                    Wave::Push(x) => {
                        stack.push(x)
                    },
                    Wave::Mult => {
                        let x = pop(&mut stack);
                        let y = pop(&mut stack);
                        stack.push(x * y);
                    }
                    _ => unimplemented!(),
                }
            }

            // eprintln!("[{idx}] Channel leaving over stack {stack:#?}");
        }

        stdout()
            .write(&[((out / 2.0 + 0.5) * 255.0) as u8])
            .unwrap();
        //
        let elapsed = start.elapsed();
        let end = Instant::now();
        while end.elapsed().lt(&wavelength.clone().saturating_sub(elapsed)) {}
        sample += 1;
    }
}
