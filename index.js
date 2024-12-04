const fs = require('fs');
const path = require('path');

// Read the input JSON file
const inputPath = path.join(__dirname, 'input.json');
const outputPath = path.join(__dirname, 'input_fixed.json');

try {
    // Read the original input.json
    const rawData = fs.readFileSync(inputPath, 'utf8');
    const inputJson = JSON.parse(rawData);

    // The receipt field is a stringified JSON, so parse it
    const receiptJson = JSON.parse(inputJson.receipt);

    // Create new object with parsed receipt
    const fixedJson = {
        ...inputJson,
        receipt: receiptJson
    };

    // Write the fixed JSON with proper formatting
    fs.writeFileSync(
        outputPath,
        JSON.stringify(fixedJson, null, 2),
        'utf8'
    );

    console.log('Successfully created fixed JSON at:', outputPath);
    console.log('\nOriginal input size:', rawData.length);
    console.log('Fixed input size:', JSON.stringify(fixedJson).length);
    
    // Validate that the fixed JSON can be re-stringified without issues
    const testParse = JSON.parse(JSON.stringify(fixedJson));
    console.log('\nValidation successful: JSON can be properly stringified and parsed');
    
} catch (error) {
    console.error('Error processing JSON:', error.message);
    process.exit(1);
}