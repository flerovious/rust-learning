# Simple Rust Calculator
A simple calculator web application written in Rust. Built using actix-web.

# Endpoints
## GET /
Returns welcome message

## GET /add/{a}/{b}
Returns the sum of two numbers with `a + b`.

## GET /sub/{a}/{b}
Returns the difference of two numbers with `a - b`.

## GET /mul/{a}/{b}
Returns the product of two numbers with `a * b`.

## GET /div/{a}/{b}
Returns the quotient of two numbers with `a / b`.