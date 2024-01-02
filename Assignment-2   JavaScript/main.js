(() => {
    let arr1 = [1, 2, 3, 4, 5];
    
    first_element = arr1[0];
    arr1 = arr1.slice(1);
    
    function2(first_element, arr1);
})();


function function2(a, arr) 
{
    let arr2 = [6, 7, 8, 9];

    arr2 = arr2.concat(...arr);
    arr2.unshift(a);
    console.log(arr2);

    const sum = arr2.reduce((acc, i) => acc + i);
    console.log("Sum is:", sum);
    sum > 30 ? console.log("Sum is greater than 30!") : console.log("Sum is less than 30!");

    let promise = new Promise((resolve , reject) => {
        let sum = arr2.reduce((e , sum)=>e + sum)
        sum > 30 ? resolve(sum) : reject();
    })

    promise.then((e)=>{
        fetch(`https://jsonplaceholder.typicode.com/photos?_limit=${e}`)
        .then(response => response.json())
        .then(images => {
            for(let i = 0 ; i < images.length ; i++){
                console.log(i+1 , images[i].url);

                const imgElement = document.createElement('img');
                imgElement.src = images[i].url;
                imgElement.width = 250;
                imgElement.height = 250;
                document.getElementById("content").appendChild(imgElement);
            }
        })
    }).catch(()=>{
        console.log("Error fetching the images!")
    })
}