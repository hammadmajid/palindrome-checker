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

	// convert string to lowercase
	int input_len = strlen(original_input);
	for (int i = 0; i < input_len; i++)
	{
		original_input[i] = tolower(original_input[i]);
	}

	// create a copy of original string
	char reversed_input[LEN];
	strcpy(reversed_input, original_input);

	// reverse the string
	for (int j = input_len; j >= 0; j--)
	{
		// TODO: reverse the string
	}

	// strcmp will return 0 if both strings are same;
	int is_same = 0;
	if (strcmp(original_input, reversed_input) == is_same)
		printf("Orginal: %s\nReversed: %s\nIs Palindrome\n", original_input, reversed_input);
	else
		printf("Orginal: %s\nReversed: %s\nIs Not Palindrome\n", original_input, reversed_input);
	return 0;
}