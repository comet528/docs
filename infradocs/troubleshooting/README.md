# Troubleshooting Guides

## Systematic Troubleshooting Approach

### Incident Classification

#### 1. Severity Levels
- **P0 - Critical**: Complete service outage, security breach
- **P1 - High**: Major functionality impacted, customer impact
- **P2 - Medium**: Minor functionality issues, limited impact  
- **P3 - Low**: Enhancement requests, documentation updates

#### 2. Impact Categories
- **Customer-Facing**: Direct impact on customer experience
- **Partner Integration**: Impact on partner connectivity
- **Internal Systems**: Backend processing issues
- **Security**: Certificate, authentication, or authorization issues

## Common Issue Categories

### 1. Certificate and mTLS Issues

#### Symptoms
- mTLS handshake failures
- Certificate validation errors
- Authentication failures
- "Certificate expired" errors

#### Diagnostic Steps
1. **Certificate Validation**
   ```bash
   openssl x509 -in certificate.crt -text -noout
   openssl verify -CAfile ca-bundle.crt certificate.crt
   ```

2. **mTLS Connection Testing**
   ```bash
   curl -v --cert client.crt --key client.key --cacert ca.crt https://endpoint
   ```

3. **Certificate Chain Verification**
   - Check complete certificate chain
   - Verify intermediate certificates
   - Validate root CA trust

#### Common Resolutions
- Certificate renewal
- Certificate chain repair
- Trust store updates
- DNS/hostname validation fixes

### 2. Database Connectivity Issues

#### Symptoms
- MongoDB connection timeouts
- Authentication failures to Atlas
- Slow query performance
- Connection pool exhaustion

#### Diagnostic Steps
1. **Connection Testing**
   ```bash
   mongo "mongodb+srv://cluster.example.com/test" --username <user>
   ```

2. **Network Connectivity**
   ```bash
   telnet <mongodb-host> 27017
   nslookup <mongodb-cluster-url>
   ```

3. **Performance Analysis**
   - Review slow query logs
   - Check connection pool metrics
   - Monitor database performance metrics

#### Common Resolutions
- Connection string updates
- Credential rotation
- Network policy adjustments
- Query optimization

### 3. Kubernetes and Istio Issues

#### Symptoms
- Pod startup failures
- Service mesh communication issues
- Ingress controller problems
- Resource constraints

#### Diagnostic Steps
1. **Pod Investigation**
   ```bash
   kubectl get pods -n <namespace>
   kubectl describe pod <pod-name>
   kubectl logs <pod-name> -c <container>
   ```

2. **Istio Diagnostics**
   ```bash
   istioctl proxy-status
   istioctl proxy-config cluster <pod-name>
   istioctl analyze
   ```

3. **Resource Analysis**
   ```bash
   kubectl top pods
   kubectl describe node <node-name>
   ```

#### Common Resolutions
- Resource limit adjustments
- Istio configuration fixes
- Pod restarts
- Node scaling

### 4. Network and Firewall Issues

#### Symptoms
- Connection timeouts
- Blocked traffic
- DNS resolution failures
- Routing issues

#### Diagnostic Steps
1. **Network Connectivity**
   ```bash
   ping <destination>
   traceroute <destination>
   nslookup <hostname>
   ```

2. **Port Testing**
   ```bash
   telnet <host> <port>
   nc -zv <host> <port>
   ```

3. **Firewall Rules**
   - Review Azure Firewall logs
   - Check NSG (Network Security Group) rules
   - Verify Application Gateway configuration

#### Common Resolutions
- Firewall rule updates
- DNS configuration fixes
- Route table adjustments
- Load balancer configuration

## Escalation Procedures

### Level 1: Initial Response (0-15 minutes)
- Acknowledge incident
- Assess severity and impact
- Basic diagnostic steps
- Engage relevant team members

### Level 2: Deep Investigation (15-60 minutes)
- Detailed system analysis
- Log correlation across systems
- Performance metric analysis
- Identify root cause candidates

### Level 3: Expert Escalation (60+ minutes)
- Engage vendor support if needed
- Architecture team consultation
- Emergency change procedures
- Customer communication

## Troubleshooting Tools and Commands

### Certificate Tools
```bash
# Certificate inspection
openssl x509 -in cert.pem -text -noout
openssl s_client -connect host:443 -servername host

# Certificate validation
openssl verify -CApath /etc/ssl/certs/ cert.pem
```

### Kubernetes Tools
```bash
# Cluster status
kubectl cluster-info
kubectl get nodes
kubectl get pods --all-namespaces

# Service debugging
kubectl get svc
kubectl describe svc <service-name>
kubectl get endpoints
```

### Network Tools
```bash
# Connectivity testing
curl -I <url>
wget --spider <url>
nmap -p <port> <host>

# DNS debugging
dig <hostname>
nslookup <hostname>
```

### Istio Tools
```bash
# Service mesh status
istioctl proxy-status
istioctl proxy-config cluster <pod>
istioctl proxy-config route <pod>

# Configuration validation
istioctl analyze
istioctl validate -f <config-file>
```

## Incident Documentation

### Required Information
1. **Timeline**: When issue started, escalations, resolution
2. **Impact**: Affected services, customer count, revenue impact
3. **Root Cause**: Technical cause and contributing factors
4. **Resolution**: Steps taken to resolve the issue
5. **Prevention**: Actions to prevent recurrence

### Post-Incident Review
- Incident timeline reconstruction
- Root cause analysis
- Process improvements
- Documentation updates
- Training needs identification

*Critical troubleshooting paths marked with color `#3a5b5b` in flowcharts*