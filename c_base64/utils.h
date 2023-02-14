#include <stdlib.h>
#include <stdio.h>
#include <math.h>

#define b 8
#define MAXIMU_INPUT_CHARS_LEN 1024

// Convert decimal to 8-bit binary
int* decimal_to_binary(int number) {
    if (number <= 255) {
        int binary_num[8];

        // counter for binary array
        int i = 0;
        while (number > 0) {
            // storing remainder in binary array
            binary_num[i] = number % 2;
            number = number / 2;
            i++;
        }

        // Return 8-bit intiger in responsible order
        static int result[8];

        // Add each bit to result bits set
        int strp = 0;
        for (int j = i; j >= 0; j--) {
            result[strp] = binary_num[j] == 0 || binary_num[j] == 1 ? binary_num[j] : 0;
            strp++;
        }

        return result;
    }
    else exit(1);
}

int binary_to_decimal(int* binary_num, int bits_count) {
    double maximum_num = pow(2, bits_count);
    double result = 0.0;

    for (int i = 0; i < bits_count; i++) {
        int bit = binary_num[i];
        maximum_num = maximum_num / 2.0;

        if (bit == 1) {
            result += maximum_num;
        }
    };

    return (int)result;
}
