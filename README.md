## Milestone 1 Reflection
1. learned the basics of rust (eg. syntax, data types, control flow, functions, etc.)
2. learned how to use the cargo package manager


## Milesstone 2 Reflection
![Commit 2 screen capture](/assets/milestone2.png)

## Milestone 3 Reflection
![Commit 3 screen capture](/assets/milestone3.png)
for this milestone i am splitting the responses based on the request path with a simple conditionals. i also refactored the code to make it more readable and easier to maintain as per the rust book's best practices in writing rust codes.

## Milestone 4 Reflection
the /sleep endpoint is intentionally delayed because of the thread::sleep function. this is bad because the server is a single-thread server. if it wasnt, the server could process other endpoints even if another endpoint is delayed. on my case the /sleep request causes the server to wait 10 seconds to process another request, this wouldn't happen with a multithreaded server.

