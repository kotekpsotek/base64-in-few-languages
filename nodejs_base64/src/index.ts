import rdln from "readline"
import { stdin, stdout } from "process";
import { performance as prf } from "perf_hooks";

type Encoded = string;
class Base64 {
    static Base64Alphabet = [
        [0, 'A'],
        [1, 'B'],
        [2, 'C'],
        [3, 'D'],
        [4, 'E'],
        [5, 'F'],
        [6, 'G'],
        [7, 'H'],
        [8, 'I'],
        [9, 'J'],
        [10, 'K'],
        [11, 'L'],
        [12, 'M'],
        [13, 'N'],
        [14, 'O'],
        [15, 'P'],
        [16, 'Q'],
        [17, 'R'],
        [18, 'S'],
        [19, 'T'],
        [20, 'U'],
        [21, 'V'],
        [22, 'W'],
        [23, 'X'],
        [24, 'Y'],
        [25, 'Z'],
        [26, 'a'],
        [27, 'b'],
        [28, 'c'],
        [29, 'd'],
        [30, 'e'],
        [31, 'f'],
        [32, 'g'],
        [33, 'h'],
        [34, 'i'],
        [35, 'j'],
        [36, 'k'],
        [37, 'l'],
        [38, 'm'],
        [39, 'n'],
        [40, 'o'],
        [41, 'p'],
        [42, 'q'],
        [43, 'r'],
        [44, 's'],
        [45, 't'],
        [46, 'u'],
        [47, 'v'],
        [48, 'w'],
        [49, 'x'],
        [50, 'y'],
        [51, 'z'],
        [52, '0'],
        [53, '1'],
        [54, '2'],
        [55, '3'],
        [56, '4'],
        [57, '5'],
        [58, '6'],
        [59, '7'],
        [60, '8'],
        [61, '9'],
        [62, '+'],
        [63, '/'],
        [64, '=']
    ];
    
    /**
     * @param toEncode - characters which should be encoded to base64
     * @returns 
     * Enocode utf-8 characters sentence to base64 format
    */
    static encode(toEncode: string): Encoded {
        let result: string = "";

        let binaryBytesChain: string = "";
        
        // Fill binaryVhain with 8 digits binary number
        const bytes = Buffer.from(toEncode, "utf-8");
        for (let [_, value] of bytes.entries()) {
            let charInBinary = value.toString(2);

            if (charInBinary.trim().length < 8) {
                const lackingZeros = Base64.refill0encode(charInBinary, 8)
                charInBinary = lackingZeros.concat(charInBinary)
            }
            // Larger then 8 bits can't be due to Buffer.from() method working scheme which create numbers in extent 0 - 255

            binaryBytesChain += charInBinary
        }

        // Insert characters from base64 alphabet
        const slices6 = binaryBytesChain.match(/\d{0,5}\S/g) || [];
        for (let sliceBinary of slices6) {
            if (sliceBinary.length < 6 && sliceBinary.includes("1")) {
                const refill0 = Base64.refill0encode(sliceBinary, 6);
                sliceBinary = sliceBinary.concat(refill0);
            } else if (sliceBinary.length < 6 && !sliceBinary.includes("1")) {
                continue;
            }
            
            const decimalNumber = parseInt(sliceBinary, 2);
            if (decimalNumber < 64) {
                const base64alpChar = Base64.Base64Alphabet.find(([number, character]) => {
                    return number == decimalNumber
                })?.[1];
    
                if (base64alpChar) {
                    result += base64alpChar
                } else throw new Error();
            }
        }

        // Insert maximum 2 base64 paddings
        const paddingsNeeded = binaryBytesChain.length % 3;
        if (paddingsNeeded != slices6.length) {
            const pdgs: string[] = [];
            for (let i = 0; i < paddingsNeeded; i++) {
                pdgs.push("=")
            }
            result = result.concat(pdgs.join(""))
        }

        // return result
        return result;
    }

    private static refill0encode(binarySentence: string, toNumber: number) {
        const lackingZerosC = toNumber - binarySentence.length;
        let lackingZeros: string = "";

        for (let i = 0; i < lackingZerosC; i++)  {
            lackingZeros += "0";
        }

        return lackingZeros;
    }
}

const rdli = rdln.createInterface({ output: stdout, input: stdin });

function question() {
    rdli.question("Data to encode/decode: ", answer => {
        const startTime = prf.now();
        const encoded = Base64.encode(answer.trim());
        console.log(encoded)
    })
}
question()
