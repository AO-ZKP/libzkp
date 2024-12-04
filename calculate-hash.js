const fs = require('fs');
const path = require('path');
const { keccak256 } = require('ethereum-cryptography/keccak');
const { hexlify } = require('@ethersproject/bytes');

function calculateJournalHash(inputPath) {
    try {
        // Read and parse the input JSON file
        const rawData = fs.readFileSync(inputPath, 'utf8');
        const inputJson = JSON.parse(rawData);

        // If receipt is a string (from input.json), parse it
        let receiptData;
        if (typeof inputJson.receipt === 'string') {
            receiptData = JSON.parse(inputJson.receipt);
        } else {
            // If receipt is already an object (from input_fixed.json)
            receiptData = inputJson.receipt;
        }

        // Get the journal bytes from the receipt
        const journalBytes = new Uint8Array(receiptData.journal.bytes);

        // Calculate keccak256 hash
        const hash = keccak256(journalBytes);

        // Convert hash to hex string with 0x prefix
        const hashHex = '0x' + Buffer.from(hash).toString('hex');

        // Create response object
        const response = {
            error: '',
            hash: hashHex
        };

        // Print the result
        console.log('Hash calculation successful!');
        console.log(JSON.stringify(response, null, 2));

        return response;
    } catch (error) {
        const errorResponse = {
            error: `Error: ${error.message}`,
            hash: ''
        };
        console.error('Hash calculation failed!');
        console.error(JSON.stringify(errorResponse, null, 2));
        return errorResponse;
    }
}

// Check if input file path is provided
const inputFile = process.argv[2] || 'input.json';  // Changed default to input.json
const inputPath = path.resolve(__dirname, inputFile);

// Calculate hash
calculateJournalHash(inputPath);