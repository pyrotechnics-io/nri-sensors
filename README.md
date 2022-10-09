# New Relic Infrastructure LM Sensors

## Overview
This is an on-host integration with New Relic for getting temperature readings via the lm-sensors package.

## Quickstart

### Steps
- Install New Relic One along with the Linux infrastructure agent
- Install lm-sensors on Linux
  ```
  sudo apt install lm-sensors libsensors4-dev
  ```
- Build the code
  ```
  git clone https://github.com/pyrotechnics-io/nri-sensors.git
  cd nri-sensors
  cargo build --release
  ```
- Install the nri-sensors binary
  ```
  sudo install target/release/nri-sensors /var/db/newrelic-infra/custom-integrations/
  ```
- Create an on-host integration configuration in  /etc/newrelic-infra/integrations.d/
  ```
  integrations:
  - name: nri-sensors
    interval: 30s
  ```
- Re-launch the infra agent
  ```
  sudo systemctl restart newrelic-infra
  ```
- Go to NR1 and run the following query (and make a dashboard as you prefer):
  ```
  SELECT latest(temp) FROM lmsensors FACET device WHERE hostname = 'myhostname' SINCE 1 hour ago TIMESERIES 1 minute 
  ```
