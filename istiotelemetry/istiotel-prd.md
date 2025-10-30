
Objective
Create me a training course and guide to get me running with istio telemetry.
I have 11 services in a namespace that currently use istio
The 10 services talk to up to 4 external endpoints, and services call between services

I'd like to understand all the incoming calls to my namespace - how long they took to complete, status code and where they came from which is likely recording the x-cert-dn header.
I'l like to understand all the outgoing calls from my namespce to keystore.sandbox.directory.openfinance.ae, mtls.keystore.sandbox.directory, *.mongodb.net, gateway.mypartner.com
I'd also like to understand duration, caller, payload size, and status code for all calls between services.
If possible I'd like to trace calls across my namespace so I can graph out call paths and timings, not bothered with content, just who's calling who in what order how long it took and the response code and when.

Once I have this level of teletry I can put a bit of load on my service and begin to understand where time is taken.

Additional, if using prometheous or grahapna is the best way of examining this data, I don't have that in my mesh - the metrics are typically exported to somewhere i don't have access. So I am able to port forward locally, or run some form of 'collector' in namespace and port forward to that.

I expect a simple course which would take a few hours which helps me work through examples of creating the appropriate yaml, deploying and gathering the stats so that the visibility I need is easily accssible.

I can run things locally from scratch - i alreday have istio with kind, i would prefer to deploy straight into my uat environment so that when its working i can benefit from the results immediatly.

Answer: see `istiotelemetry/istiotel-training.md` for a focused half‑day course with hands‑on labs and ready‑to‑apply manifests under `istiotelemetry/examples/`.
