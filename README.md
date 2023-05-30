## Creating an Api-Gateway using Rust

### How to run:
 - Get into /src folder 
 - Run cargo run main.rs
 - Run: .\main 

### About:
Mediation Server

To integrate the other services, think about defining a basic sending rule so that we know which services are being requested before we redirect, basically, for every sending we expect the object to have the following body:
```
Example:
{
   “type”: “investment”, =:> the service;
   “action”: “ “, =:> the query you want;
   “content”: “ “ =:> the content if the requested route needs it (Optional Field).
}
````

How it works: the requested type will be searched in a hashMap with key and value, where the key is just the type and the value is the address of the service. The action we will use to concatenate with the obtained address:

Ex: http://localhost:8000/action

Note: For now we have defined only the GET, POST and DELETE methods.

Also, it did the authentication part.
