#!/bin/bash
TEST_NAME=mme

# TODO: configure test traffic input
# PORT_OPTIONS="dpdk:eth_pcap0,rx_pcap=data/in.pcap,tx_pcap=tmp/out.pcap"
PORT_OPTIONS="dpdk:eth_pcap0,iface=enp0s8"

echo "../../build.sh run $TEST_NAME -p $PORT_OPTIONS -c 1"
../../build.sh run $TEST_NAME -p $PORT_OPTIONS -c 1

# TODO: fill in rest of test