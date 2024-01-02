// Storing and displaying all the element of array using Recursion

let array = [28, 9, 42, 78, 12, 97, 56, 43];
let new_array = [];

function storeElements(arr, index) {
    if (index === arr.length) {
        return new_array;
    }

    new_array.push(arr[index]);
    storeElements(arr, index+1);
}


function printElements(arr, index) {
    if (index === arr.length) {
        return; 
    }

    console.log(arr[index]);
    printElements(arr, index+1);
}


storeElements(array, 0);
console.log('Elements successfully stored in array!');

console.log('Elements stored in the array are:');
printElements(array, 0);