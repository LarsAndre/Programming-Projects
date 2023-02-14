# Javascript Notes

## Important tips

1. Placing scripts at the bottom of the `<body>` element **improves the display speed**, because script interpretation slows down the display.



### External Javascript
1. External scripts cannot contain `<script>` tags.
2. External scripts are practical when the same code is used in many different web pages.
3. Placing scripts in external files has some advantages:
   - It separates HTML and code
   - It makes HTML and JavaScript easier to read and maintain
   - Cached JavaScript files can speed up page loads
4. An external script can be referenced in 3 different ways:
   1. With a full URL (a full web address)
   2. With a file path (like /js/)
   3. Without any path
```js
// 1.
<script src="https://www.w3schools.com/js/myScript.js"></script>
// 2.
<script src="/js/myScript.js"></script>
// 3.
<script src="myScript.js"></script>
```

### Javascript Output
1. JavaScript can "display" data in different ways:
   - Writing into an HTML element, using ``innerHTML``.
   - Writing into the HTML output using `document.write()`. **Using document.write() after an HTML document is loaded, will delete all existing HTML**
   - Writing into an alert box, using ``window.alert()``.
   - Writing into the browser console, using ``console.log()``.







## General Knowledge

The `<script>` tag: This is where javascript is inserted. Scripts can be placed in the HTML `<body>`, `<head>` or both.
```html
<!DOCTYPE html>
<html>
<body>

<h2>JavaScript in Body</h2>

<p id="demo"></p>

<script>
document.getElementById("demo").innerHTML = "My First JavaScript";
</script>

</body>
</html> 
```

Javascript `function` is a block of javascript code that can be executed when "called" for, for example when an *event* occurs (clicking a button) as in the example below:
```html
<!DOCTYPE html>
<html>
<head>
<script>
function myFunction() {
  document.getElementById("demo").innerHTML = "Paragraph changed.";
}
</script>
</head>
<body>
<h2>Demo JavaScript in Head</h2>

<p id="demo">A Paragraph</p>
<button type="button" onclick="myFunction()">Try it</button>

</body>
</html>
```



## Methods

- `getElementById()`: Returns an element object representing the element whose id property matches the specified string 

## JavaScript Syntax

```js	
// How to create variables
var x;
let y;

// How to use variables
x = 5;
y = 6;
let z = x + y;
```

### JavaScript Values
The javaScript values are of 2 types:
1. Fixed values -> Literals
2. Variable values -> Variables

#### JavaScript Literals
A literal is a fixed value that you literally provide in your script. For example, the string "Hello World" is a literal. The number 5 is also a literal.

#### JavaScript Variables
A variable is a named storage for data. We can use variables to store data.

#### JavaScript Identifiers
An identifier is a name. In JavaScript, identifiers are used to name variables (and keywords, and functions, and labels).

A JavaScript name must begin with:
- A letter (A-Z or a-z)
- A dollar sign ($)
- Or an underscore (_)

Numbers are not allowed to be the first character.

#### JavaScript and Camel Case
Hyphens are not allowed in JavaScript because they are reserved for subtractions. Camel case is the practice of writing compound words or phrases such that each word or abbreviation in the middle of the phrase begins with a capital letter, with no intervening spaces or punctuation.

No-go: `my-first-variable`
Underscore: `my_first_variable`
Camel case: `myFirstVariable`
