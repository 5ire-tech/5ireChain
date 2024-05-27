# 5irechain Blockchain Node Scripts
This repository contains scripts to manage 5irechain blockchain nodes inside Docker containers. Below are the available scripts:

1. build.sh
This script creates a Docker image for the 5irechain blockchain node. By default, it creates an image for the QA environment. Use the -e flag to specify a different environment.

Usage:  
<<<<<<< HEAD
```bash
./build.sh [-e <environment>] [-h | --help]  
```
=======
bash ./build.sh [-e <environment>] [-h | --help]  
>>>>>>> cdb6b8a (updated validator nodes flags, added scripts readme & added upload.sh)

2. run.sh
This script runs a Docker container for the 5irechain node in one of the following modes: fullnode, validator, or archivenode. Only one mode should be passed using flags -f | --fullnode, -v | --validator, or -a | --archivenode. Additionally, provide the bootnode IP using -b | --bootnode flag and the bootnode ID using -bid | --bootnode-id flag. You can use the -e flag to specify the environment. By default, it will look for an existing image. Use the --build flag to create a new image before running.

Usage:  
<<<<<<< HEAD
```bash
./run.sh [-f | --fullnode] [-v | --validator] [-a | --archivenode] [-b <bootnode_ip>] [-bid <bootnode_id>] [-e <environment>] [--build] [-h | --help]  
```
=======
bash ./run.sh [-f | --fullnode] [-v | --validator] [-a | --archivenode] [-b <bootnode_ip>] [-bid <bootnode_id>] [-e <environment>] [--build] [-h | --help]  
>>>>>>> cdb6b8a (updated validator nodes flags, added scripts readme & added upload.sh)

3. upload.sh
This script uploads the Docker image to the respective ECR repository based on the environment selected. Use the -e flag to specify the environment. Use the --build flag to build a new Docker image before uploading. For the QA environment, specify the AWS profile using the --profile flag. By default, it will use the default profile. For the Thunder (testnet) environment, the image is uploaded to the public 5irechain repo.

<<<<<<< HEAD
Usage:
```bash  
./upload.sh [-e <environment>] [--build] [--profile <aws_profile>] [-h | --help]  
```
=======
Usage:  
bash ./upload.sh [-e <environment>] [--build] [--profile <aws_profile>] [-h | --help]  
>>>>>>> cdb6b8a (updated validator nodes flags, added scripts readme & added upload.sh)

Note: All scripts should be executed from inside the docker folder.

Please ensure that you have the necessary permissions and configurations set up before running these scripts. For any additional information or assistance, refer to the documentation or contact the devops team.