const fs = require('fs');
const { generate_table } = require('./pkg/wasm_bindgen.js');

(function make_table() {
    fs.readFile('./Table_data.json', 'utf8', (err, data) => {
        if (err) {
          console.error('Error:', err);
          return;
        }

        try {
          // const jsonData = JSON.parse(data);
          const result = generate_table(data);

          fs.writeFile('./Table_output.json', result, 'utf8', (err) => {
            if (err) {
              console.error('Error writing to output file:', err);
              return;
            }
            console.log('Table generated successfully !!');
          });
        }
        catch(err) {
          console.log('Error generating table:', err);
        }
    });
})();