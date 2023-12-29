// Javascript program to calculate the combination

function factorial(n) 
{
    if (n===0 || n===1)
      return 1;
    else
      return n * factorial(n - 1);
}
  
function combination(n, r)
{
    if (n < r)
        return 0;
    else
      return factorial(n) / (factorial(r) * factorial(n - r));
}
  

const n = 7;
const r = 2;

console.log(`Combination(${n}, ${r}) = ${combination(n, r)}`);