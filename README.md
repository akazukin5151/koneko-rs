In less than a week, I suddenly decided to rewrite koneko in rust for some reason.

But why? Why rust? Logically speaking, the bottleneck of koneko is network speed, so it doesn't make much sense to optimise the speed of CPU intensive parts much. The second bottleneck is pixcat, which is written in python. To use it in rust is to either port it, or use it with PyO3 (negating the speed benefits of rust). So why? I think it's because once I successfully wrote code with the rust compiler and borrow checker, I don't want to go back to an uncompiled, dynamic language.
