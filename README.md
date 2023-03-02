# gentle and kind - local k8s cluster bootstrapper
Installs and bootstraps a kind cluster on Mac OSX with some features:

* nginx-ingress
* .test Top-Level-Domain resolution on host system
* TLS certificates with cert-manager
* monitoring stack (prometheus and loki)
* postgresql installed with port-forward

## Prerequisites
You can run `./gentle check` to see if your machine meets the prerequisites. Docker needs to be running.

## Bootstrapping
Run `./gentle bootstrap` to install the cluster.
If all runs well you have to do two additional steps:

### Import the cert
Double click the tls.crt file to add the certificate to the keychain.
Then find the certificate (**Gentle Certificate Authority**) in the "Keychain Access" app, right click the entry and choose 
"Get Info" to open the dialog (or double click the entry instead). 
Expand the "Trust" section and set "Secure Sockets Layer (SSL)" to "Always trust".

### Set the resolver for .test TLD
Run
```
sudo bash -c 'mkdir -p /etc/resolver && echo "nameserver 127.0.0.1" > /etc/resolver/test'
```