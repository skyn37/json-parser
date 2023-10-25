# Build Your Own JSON Parser

This challenge involves building your own JSON parser, a valuable exercise for learning about parsing techniques applicable to various data formats.

Parsing is typically divided into two stages: lexical analysis and syntactic analysis. Lexical analysis involves dividing a character sequence into meaningful chunks called tokens, while syntactic analysis (parsing) processes the list of tokens according to a formal grammar.

For a more in-depth understanding of lexers, parsers, and compilers, you can refer to the book "Compilers: Principles, Techniques, and Tools" (commonly known as the "Dragon Book").

## The Challenge - Building a JSON Parser

JSON (JavaScript Object Notation) is a lightweight data-interchange format widely used for data transmission over the internet. It's formally defined by the IETF [here](https://tools.ietf.org/html/rfc8259) and has a simpler graphical representation [here](https://www.json.org/json-en.html).

### Step Zero

To get started, set up your preferred IDE/editor and programming language. You can download test data for the JSON parser from [here](https://www.example.com/testdata).

### Step 1

In this step, the goal is to parse a valid simple JSON object ('{}') and an invalid JSON file and correctly identify them. Build a basic lexer and parser.

Your program should report suitable messages to the standard output stream and exit with code 0 for valid input and 1 for invalid input. This convention is used in CLI tools, allowing them to be combined to create more powerful programs. Refer to the "Write Your Own `wc` Tool" challenge for insights on combining CLI tools.

Test your code against the files in the 'tests/step1' folder. Consider automating tests to facilitate repeated testing as you progress.

### Step 2

Extend the parser to handle a simple JSON object containing string keys and string values, like:

```
{"key": "value"}
```

Test your code against the files in the 'tests/step2' folder.

### Step 3

Extend the parser to handle a JSON object containing string, numeric, boolean, and null values, like:

```
{
  "key1": true,
  "key2": false,
  "key3": null,
  "key4": "value",
  "key5": 101
}
```

Test your code against the files in the 'tests/step3' folder.

### Step 4

Further extend the parser to handle a JSON object with object and array values, such as:

```
{
  "key": "value",
  "key-n": 101,
  "key-o": {},
  "key-l": []
}
```

Test your code against the files in the 'tests/step4' folder.

### Step 5

Add your own tests to ensure your parser can handle valid JSON and provide useful error messages for invalid JSON. Once you're confident in your parser, consider running it against the test suite found [here](http://www.json.org/JSON_checker/test.zip).

---

This challenge allows you to explore parsing techniques and build a functional JSON parser. Enjoy the journey!