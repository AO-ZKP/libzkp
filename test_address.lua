-- test_address.lua

-- Base64URL decode table
local b64URL = {
    ['A']=0,['B']=1,['C']=2,['D']=3,['E']=4,['F']=5,['G']=6,['H']=7,['I']=8,
    ['J']=9,['K']=10,['L']=11,['M']=12,['N']=13,['O']=14,['P']=15,['Q']=16,
    ['R']=17,['S']=18,['T']=19,['U']=20,['V']=21,['W']=22,['X']=23,['Y']=24,
    ['Z']=25,['a']=26,['b']=27,['c']=28,['d']=29,['e']=30,['f']=31,['g']=32,
    ['h']=33,['i']=34,['j']=35,['k']=36,['l']=37,['m']=38,['n']=39,['o']=40,
    ['p']=41,['q']=42,['r']=43,['s']=44,['t']=45,['u']=46,['v']=47,['w']=48,
    ['x']=49,['y']=50,['z']=51,['0']=52,['1']=53,['2']=54,['3']=55,['4']=56,
    ['5']=57,['6']=58,['7']=59,['8']=60,['9']=61,['-']=62,['_']=63
}

-- Helper function to convert bytes to hex
local function bytesToHex(bytes)
    local hex = ""
    for i = 1, #bytes do
        hex = hex .. string.format("%02x", bytes[i])
    end
    return hex
end

-- Base64URL decode without bit operations
local function base64UrlToBytes(str)
    local bytes = {}
    
    -- Add padding if needed
    local padding = (4 - (#str % 4)) % 4
    str = str .. string.rep("A", padding)  -- 'A' will be ignored in decoding
    
    for i = 1, #str, 4 do
        local n1 = b64URL[str:sub(i, i)] or 0
        local n2 = b64URL[str:sub(i+1, i+1)] or 0
        local n3 = b64URL[str:sub(i+2, i+2)] or 0
        local n4 = b64URL[str:sub(i+3, i+3)] or 0
        
        local b1 = (n1 * 4) + math.floor(n2 / 16)
        local b2 = ((n2 % 16) * 16) + math.floor(n3 / 4)
        local b3 = ((n3 % 4) * 64) + n4
        
        bytes[#bytes + 1] = b1
        if i+1 <= #str - padding then bytes[#bytes + 1] = b2 end
        if i+2 <= #str - padding then bytes[#bytes + 1] = b3 end
    end
    
    return bytes
end

-- Convert Arweave address to Ethereum address
local function arweaveToEthereum(arweaveAddr)
    local bytes = base64UrlToBytes(arweaveAddr)
    local ethBytes = {}
    
    -- Take last 20 bytes but start from the correct position
    local startIndex = #bytes - 20  -- Changed from -19 to -20
    for i = startIndex, #bytes - 1 do  -- Changed to #bytes - 1 to exclude the last byte
        table.insert(ethBytes, bytes[i])
    end
    
    -- Debug output
    print("Decoded bytes length:", #bytes)
    print("ETH bytes length:", #ethBytes)
    print("Full decoded hex:", bytesToHex(bytes))
    
    return "0x" .. bytesToHex(ethBytes)
end

-- Test
local arweaveAddr = "pG8tOsUrlvxgHAH7FXBXn8chb8FW5ZXOVKRyVF1wr2o"
local expectedEth = "0x1570579fc7216fc156e595ce54a472545d70af6a"
local resultEth = arweaveToEthereum(arweaveAddr)

print("Arweave address:", arweaveAddr)
print("Expected ETH:", expectedEth)
print("Result ETH:  ", resultEth)
print("Match:", resultEth == expectedEth)

assert(resultEth == expectedEth, "Ethereum address mismatch!")
print("\nAll tests passed!")