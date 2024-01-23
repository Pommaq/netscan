# Quality goals
* Maintainability over time
* Compatibility with payload sources
* Scalability

## Design
* We are going to want to separate the user interface from the business logic
* We require support for multiple payload sources. Along with custom but still reusable logic for each source since we cannot expect identical behavior


## Requirements description
A service scanner that allows scanning for open ports over tcp/udp as well as other services.
In the future I want it to support authenticated scanning for ssh as well as (optionally) other command sources (e.g. command injections). 

Portscan and service scan should need to support credentials for authentication as well as custom creds for individual scan modules. It should be possible to scan for specific services as well as list what services we can scan for. 

## Use cases
* Scan for open ports
* Scan for services
## ADRS

### ADR-1: Use a publish-subscribe pattern for controlling
#### Context
We need to design the primary method for modules to communicate. A pub-sub pattern where modules register for events and can publish events would allow all modules to run asynchronously. We need to allow blocking events however where nothing else is allowed to run while it is being completed. This is typical for systems where we have producers and consumers of information, which matches a scanner.

#### Consequences
This should allow increased maintainability by decoupling code from different modules as well as improve performance by allowing us to run everything in an asynchronous manner. 

* All modules will recieve an event queue trait objects with the methods SUBCRIBE and PUBLISH.
    * Calling SUBSCRIBE(Event) returns a Receiver<Vec<u8>> from which it can listen to events.
    * Calling PUBLISH(Event, Vec<u8>>) publishes an event with Vec<u8> as data.



## Scenarios

### 1: A scan is started
* All modules register the events they are interested in
* A scan is scheduled, an IP + settings are published to the feed.
* Portscan picks up IP/settings from feed.
* Portscan starts scan
    * Runs steps that are synchronous before publishing anything.
    * publishing found ports. 
    * Publishes a message indicating it has finished
* Servicescan picks up open ports + settings. Publishes a message indicating it has finished
* Detected services/other information is published
* Report service picks up ports from portscan as well as detected services to formulate a report. Waits for both servicescan and portscan to report being finished.

