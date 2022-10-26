#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define LEN 256

int main()
{
	// read string from user
	char original_input[LEN];
	printf("Enter a string: ");
	fgets(original_input, sizeof(original_input), stdin);

	// remove unwanted \n from string
	strtok(original_input, "\n");

	// reverse the string
	int input_len = strlen(original_input);
	char reversed_input[LEN];
	int k = 0;
	for (int j = input_len - 1; j >= 0; --j)
	{
		reversed_input[k] = original_input[j];
		k++;
	}

	// compare and print the strings
	if (strcmp(original_input, reversed_input) == 0)
		printf("Orginal: %s\nReversed: %s\nIs Palindrome\n", original_input, reversed_input);
	else
		printf("Orginal: %s\nReversed: %s\nIs Not Palindrome\n", original_input, reversed_input);
	return 0;
}