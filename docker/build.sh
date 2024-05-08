#!/bin/bash

# Function to build Docker image
build_image() {
    docker build -t 5ireqa -f firechain_builder.Dockerfile ../
}

# Function to upload image to AWS ECR
upload_to_ecr() {
    # QA Account ID
    aws_account_id="392225661532"
    ecr_repo="firechain-qa"

    # Prompt for AWS region
    read -p "Enter AWS region: " aws_region

    # Prompt for AWS profile
    read -p "Enter AWS profile: " aws_profile

    # Tag image with timestamp
    timestamp=$(date +%Y%m%d%H%M%S)
    docker tag 5ireqa:latest "$aws_account_id.dkr.ecr.$aws_region.amazonaws.com/$ecr_repo:$timestamp"

    # Authenticate Docker client with AWS ECR
    aws ecr get-login-password --region "$aws_region" --profile "$aws_profile" | docker login --username AWS --password-stdin "$aws_account_id.dkr.ecr.$aws_region.amazonaws.com"

    # Push image to AWS ECR
    docker push "$aws_account_id.dkr.ecr.$aws_region.amazonaws.com/$ecr_repo:$timestamp"
}

# Main function
main() {
    # Build Docker image
    build_image

    # Ask user if they want to upload to AWS ECR
    read -p "Do you want to upload the image to AWS ECR? (yes/no): " upload_choice

    if [ "$upload_choice" == "yes" ]; then

        # Upload image to AWS ECR
        upload_to_ecr
    else
        echo "Skipping upload to AWS ECR."
    fi
}

# Execute main function
main
