In this challenge, the task is to debug the existing code to successfully execute all provided test files.

---

Given two dates each in the format *dd-mm-yyyy*, you have to find the number of lucky dates between them (inclusive). To see if a date is lucky,

- Firstly, sequentially concatinate the date, month and year, into a new integer erasing the leading zeroes.
- Now if is divisible by either or , then we call the date a lucky date.

For example, let's take the date "02-08-2024". After concatinating the day, month and year, we get \= 2082024. As is divisible by so the date "02-08-2024" is called a lucky date.

Debug the given function `findPrimeDates` and/or other lines of code, to find the correct lucky dates from the given input.

**Note:** You can modify at most *five* lines in the given code and you cannot add or remove lines to the code.

*To restore the original code, click on the icon to the right of the language selector.*

**Input Format**

The only line of the input contains two strings and denoting the two dates following the format *dd-mm-yyyy*. Consider, is the day number, is the month number and is the year number.

Note: Here means January, means February, means March and so on and all the dates follow the standard structure of English calender including the leap year.

**Constraints**

  
  

**Output Format**

For each test cases, print a single integer the number of lucky dates between and in a single line.