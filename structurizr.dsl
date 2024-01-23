workspace {

    model {
        user = person "User"
        
        softwareSystem = softwareSystem "netscan" {
            
            pubsub = container "Pub-Sub interface"
            ui = container "User interface" {
                this -> pubsub "Schedules scans"
                this -> pubsub "Registers for log messages"
                this -> pubsub "Registers for open ports"
                this -> pubsub "Registers for found services"
            }
            portscan = container "Portscanner" {
                this -> pubsub "registers for scheduled scans"
                this -> pubsub "Registers open ports"
            }
            
            servicescan = container "Service Scanner" {
                this -> pubsub "Registers for open ports"
                this -> pubsub "Registers found services"
            }
        }
    }

    views {
        systemContext softwareSystem {
            include *
            autolayout lr
        }

        container softwareSystem {
            include *
            autolayout lr
        }

        theme default
    }

}