measure-command { start-process .\target/release/mandelbrot -ArgumentList "mandel.png 4000x3000 -1.20,0.35 -1,0.20 p"  -Wait }
measure-command { start-process .\target/release/mandelbrot -ArgumentList "mandel2.png 4000x3000 -1.20,0.35 -1,0.20 r"  -Wait }
