// Generating a Fibonnaci series upto 'n' terms and then finding out the sum of prime differences out of it.

function fibonacci(n, s = []) {
    if(n === 0) 
        return s;
  
    if(s.length <= 1)
        s.push(s.length);
    else
        s.push(s[s.length-1] + s[s.length-2]);
  
    return fibonacci(n-1, s);
  }


function check_prime(series)
{
    let p_series = [];
    let temp = 0;

    for(let x of series) {
        temp = 0;
        if(x >= 2) {
            for(let i=2 ; i < Math.sqrt(x) ; i++) {
                if(x%i === 0) {
                    temp = 1;
                    break;
                }
            }
            
            if(temp === 0) {
                p_series.push(x);
            }
        }
    }

    return p_series;
}

let n = 100;
let sum = 0;
let series = fibonacci(n);
let prime_series = check_prime(series);


for(let i=1 ; i<prime_series.length ; i++)
{
    let diff = prime_series[i] - prime_series[i-1];
    sum += diff;
}


console.log(`Fibonacci series up to ${n}:`, series);
console.log("===============================================================");
console.log("Prime Fibonacci series:", prime_series);
console.log("===============================================================");
console.log("Sum of differences:", sum);