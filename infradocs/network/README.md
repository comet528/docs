# Network Architecture

## Network Overview

Complex network topology supporting secure communications between customers, partners, and internal services through Azure infrastructure in the UAE region.

## Network Zones

### 1. External Zone (Internet)
- **Customer Traffic**: Incoming mTLS connections
- **Partner Traffic**: Bidirectional mTLS connections  
- **Webhook Traffic**: Outbound HTTPS to customer endpoints

### 2. Perimeter Zone (Azure Edge)
- **Web Application Firewall (WAF)**
  - DDoS protection
  - OWASP rule sets
  - Custom security rules
- **Azure Firewall**
  - Network filtering
  - Application rules
  - Threat intelligence

### 3. Application Zone (AKS)
- **Application Gateway Ingress Controller**
  - SSL termination
  - Path-based routing
  - Backend health monitoring
- **Istio Service Mesh**
  - Service-to-service mTLS
  - Traffic management
  - Security policies

### 4. Data Zone
- **MongoDB Atlas**
  - Private endpoint connectivity
  - TLS encryption in transit
  - Network peering/private link

## Traffic Flow Patterns

### Inbound Customer Traffic
```
[Customer] --mTLS--> [WAF] ---> [Azure Firewall] ---> [App Gateway] ---> [Istio Gateway] ---> [Services]
```

### Partner Integration Traffic
```
[Partner] <--mTLS--> [Azure Firewall] <---> [Istio Gateway] <---> [Services]
```

### Database Connectivity
```
[Services] --TLS--> [Private Endpoint] ---> [MongoDB Atlas]
```

### Outbound Webhooks
```
[Services] ---> [Istio Egress] ---> [Azure Firewall] --HTTPS--> [Customer Webhooks]
```

## Network Security Policies

### Firewall Rules
1. **Inbound Rules**
   - Allow customer mTLS (port 443/8443)
   - Allow partner mTLS (specific ports)
   - Deny all other inbound traffic

2. **Outbound Rules**  
   - Allow MongoDB Atlas (port 27017)
   - Allow customer webhooks (port 443)
   - Allow DNS resolution
   - Deny all other outbound traffic

### Istio Security Policies
- **Service-to-service mTLS**: Enforced for all internal communication
- **Authorization Policies**: Role-based access control
- **Network Policies**: Kubernetes native network segmentation

## Network Monitoring Requirements

### Key Metrics to Monitor
- **Connection Success Rates**: mTLS handshake success
- **Latency Metrics**: End-to-end response times
- **Throughput**: Data transfer rates
- **Error Rates**: Connection failures and timeouts

### Alerting Thresholds
- mTLS handshake failures > 1%
- Database connection latency > 100ms
- Webhook delivery failures > 5%

## Network Troubleshooting Tools

### Connectivity Testing
- **Certificate validation tools**
- **Network connectivity probes**  
- **TLS handshake analyzers**
- **Istio diagnostic tools**

### Logging and Monitoring
- **Azure Firewall logs**
- **Application Gateway access logs**
- **Istio access logs**
- **Network flow logs**

## Network Diagram Requirements

Network diagrams should show:
1. **Physical topology**: Azure resources and connections
2. **Logical topology**: Traffic flows and security boundaries  
3. **Certificate flows**: Where certificates are used
4. **Monitoring points**: Where telemetry is collected

*Critical network paths highlighted in color `#3a5b5b`*