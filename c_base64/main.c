#include <stdlib.h>
#include <stdio.h>
#include "alphabet.h"
#include "utils.h"

char* encode_to_base64(char* captured_sentence) {
    int* binary_8_bits_chain = calloc(MAXIMU_INPUT_CHARS_LEN * 8, sizeof(int));
    int last_filled_bit;

    // Emerygency program terminating
    if (binary_8_bits_chain == NULL) exit(1);

    // Get ascii character code and convert it to 8 bit binary
    for (int i = 0; i < MAXIMU_INPUT_CHARS_LEN; i++) {
        // Get ascii character from memory array block
        char character = captured_sentence[i];

        // Get ascii character code
        int ascii_code = (int)character;
        printf("%d %c", ascii_code, character);
        if (ascii_code == 0) {
            break;
        } else {
            printf("%d\n", character, ascii_code);
            
            // Returns pointer to 8 bits obtained from ascii code
            int* code_in_binary = decimal_to_binary(ascii_code);
    
            // Add obtained bites to bites chain
            for (int it = 0; it < 8; it++) {
                binary_8_bits_chain[last_filled_bit] = code_in_binary[it];
                last_filled_bit++;
            }
        }
    }

    // ALter 8 bit chain value to 6 bit binary intiger and obtain apropriate base64 character for it
    char* base64_result = calloc(MAXIMU_INPUT_CHARS_LEN * 4, sizeof(char));
    int chain_6_bits[6], added_count = 0, base64_last = 0;
    for (int i = 0; i < last_filled_bit; i++) {
        int selected_bit_from_chain = binary_8_bits_chain[i];

        // Add bit to create 6 bit chain
        chain_6_bits[added_count] = selected_bit_from_chain;
        added_count++;

        if (added_count == 6) {
            int b64_code = binary_to_decimal(chain_6_bits, 6);
            char b64_char = BASE64_ALPHABET[b64_code];

            base64_result[base64_last] = b64_char;
            base64_last++;
            added_count = 0;
        }
    }

    // complete to 6 bits
    if (added_count != 0) {
        int remainding_bits = 6 - added_count;
        
        for (int i = 0; i < remainding_bits; i++) {
            chain_6_bits[added_count] = 0;
            added_count++;
        }
        
        if (added_count == 6) {
            int b64_code = binary_to_decimal(chain_6_bits, 6);
            char b64_char = BASE64_ALPHABET[b64_code];

            base64_result[base64_last] = b64_char;
            base64_last++;
            added_count = 0;
        }
    }

    // Add remainding padding
    if (base64_last % 4 != 0) {
        int complete_with_padding_c = 4 - base64_last % 4;

        for (int p = 0; p < complete_with_padding_c; p++) {
            base64_result[base64_last] = BASE64_ALPHABET[64];
            base64_last++;
        }
    }

    return base64_result;
}

void main() {
    // Create memroy blocks to store input
    char* input_store = calloc(MAXIMU_INPUT_CHARS_LEN, sizeof(char));

    if (input_store != NULL) {
        printf("Pass sentence to encode: ");
        scanf("%[^\n]", input_store);

        // Encode to base64 output
        char* encoded = encode_to_base64(input_store);
        printf("\n\nEncoding result: %s", encoded);
    }
    else {
        printf("Couldn't allocate memory for input");
        exit(1);
    }
}