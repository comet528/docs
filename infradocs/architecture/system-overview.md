# System Architecture Overview

## System Description

Large Kubernetes system with Istio service mesh on Microsoft AKS/Azure in UAE region.

### Key Components

#### Infrastructure Layer
- **Platform**: Microsoft Azure Kubernetes Service (AKS)
- **Location**: UAE region
- **Service Mesh**: Istio
- **Ingress**: App Gateway Ingress Controller
- **Security**: Microsoft WAF, Azure Firewalls

#### Data Layer
- **Primary Database**: MongoDB Atlas
- **Connection Type**: Secured connections with TLS

#### Security & Trust Framework
- **PKI Infrastructure**: Private directory issuing all certificates
- **Certificate Types**:
  - TLS certificates
  - Signing certificates  
  - Encryption certificates
- **Usage Scope**: 
  - External client connectivity
  - External partner connectivity
  - Internal service communication

#### Application Layer
- **TypeScript Services**: 11 services
- **Scheduled Jobs**: 7 cron jobs
- **External Integrations**: Partner mTLS connections
- **Customer Notifications**: HTTPS/webhooks
- **Incoming Traffic**: mTLS from customers

## High-Level Architecture

```
[Customers] --mTLS--> [Azure WAF/Firewall] ---> [App Gateway] ---> [AKS/Istio] ---> [Services]
                                                                      |
[Partners] --mTLS--> [Azure Firewall] -------------------------> [AKS/Istio]
                                                                      |
                                                                 [MongoDB Atlas]
```

## Critical Success Factors

1. **Certificate Management**: Proper PKI operations for all TLS communications
2. **Network Connectivity**: Reliable connections to MongoDB Atlas
3. **Partner Integration**: Functioning mTLS connections with external partners
4. **Customer Communication**: Reliable webhook delivery
5. **Service Health**: All 11 TypeScript services and 7 cron jobs operational

## Color Convention
- Critical paths and components highlighted in: `#3a5b5b`