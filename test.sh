#!/bin/dunesh
echo "Hello, world!";

let is-prime = n -> {
	let result = True;
        for i in 2 to n // 2 + 1 {
                if n % i == 0 { let result = False; };
        };
        result && n > 1;
};

for i in 0 to 100 {
	echo i "is" (if (is-prime i) {"prime"} else {"composite"});
};

echo "Goodbye, world!"
