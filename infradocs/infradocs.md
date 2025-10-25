Infrastructure Documentation.

Documentation from a troubleshooting/investigation/incident response perspective.

Having clarity around what occurred (what where when how why)

We have a large k8s system with istio on microsoft aks / azure in uae
App gateway ingress controller, firewalls, microsoft WAFS

- The system relies heavily on talking to a mongod altlas database

- It also relies on a trust framework backed by  pki - a private directory issue all tls, siging and encryption certs used across the system - which include external client connectivity and external partner connectiviy

- connections to partners via mtls
- notification to customers via https/webhooks
- incoming traffic via mtls from customers
- plus 11 typescript services and 7 cron jobs

I need you do define the documentaion set required to support and troubleshoot this complex system including infrastructure and network diagrams and what each should show.

I'd like to use the documetnation set as a guide/template for collecting all the appropraite information to ensure that the entire system is supportable

