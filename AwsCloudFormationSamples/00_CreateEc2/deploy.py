#!/home/codespace/.python/current/bin/python
import os
import sys
import json
import yaml
import time
import uuid
import shutil

if not os.path.isdir("deployments"):
    os.mkdir("deployments")

deployment_id = "createEc2-" + str(uuid.uuid4())
deployment_path = os.path.join("deployments", deployment_id)
os.mkdir(deployment_path)

sys.path.append("../../scripts/")
import testbench.aws as aws

keyName = deployment_id
stackName = deployment_id
params = { "KeyName": keyName }

aws.make_ec2_key(keyName, os.path.join(deployment_path, f"{keyName}.pem"))
os.system(f"aws cloudformation create-stack --stack-name \"{stackName}\" --template-body \"file://createEc2.cform\" --parameters ParameterKey=KeyName,ParameterValue=\"{params['KeyName']}\" ")

while (True):
    time.sleep(5)
    print("Waiting for stack creation.")
    createStatus = json.loads(os.popen(f"aws cloudformation describe-stacks --stack-name \"{stackName}\"").read())
    statusCode = createStatus["Stacks"][0]["StackStatus"]
    print(statusCode)
    if  statusCode == "CREATE_COMPLETE":
        break;

resources = json.loads(os.popen(f"aws cloudformation describe-stack-resources --stack-name \"{stackName}\"").read())
with open(os.path.join(deployment_path,"resources.log"), "w") as resourcesFile:
    resourcesFile.write(yaml.dump(resources))

ec2_physical_id=""
for resource in resources["StackResources"]:
    if resource["LogicalResourceId"] == "EC2Instance":
        ec2_physical_id = resource["PhysicalResourceId"]

print(f"EC2 Physical Resource Id: {ec2_physical_id}")
ec2_resource = json.loads(os.popen(f"aws ec2 describe-instances --instance-ids {ec2_physical_id}").read())
public_dns_name = ec2_resource["Reservations"][0]["Instances"][0]["PublicDnsName"]
print(f"EC2 Public DNS Name: {public_dns_name}")

# SSH Script for the EC2 instance
sshScript = f"""
ssh -i \"{keyName}.pem\" ec2-user@{public_dns_name}
"""
with open(os.path.join(deployment_path,"ssh.sh"), "w") as sshScriptFile:
    sshScriptFile.write(sshScript)

# Cleanup script for the deployment
sshScript = \
f"""#!{shutil.which("bash")}
aws cloudformation delete-stack --stack-name \"{stackName}\"
"""
teardownPath = os.path.join(deployment_path,"teardown.sh")
with open(teardownPath, "w") as sshScriptFile:
    sshScriptFile.write(sshScript)
os.system(f"chmod u+x {teardownPath}")