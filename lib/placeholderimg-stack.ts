import * as cdk from '@aws-cdk/core';
import { RemovalPolicy } from '@aws-cdk/core';
import * as s3 from '@aws-cdk/aws-s3';
import * as s3deploy from '@aws-cdk/aws-s3-deployment';
import * as lambda from '@aws-cdk/aws-lambda';
import * as apigw2 from '@aws-cdk/aws-apigatewayv2';
import * as apigw2integrarion from '@aws-cdk/aws-apigatewayv2-integrations';
import * as cloudfront from '@aws-cdk/aws-cloudfront';
import * as acm from '@aws-cdk/aws-certificatemanager';
import { PriceClass } from '@aws-cdk/aws-cloudfront';
import { deployCfg } from "./cdk-config";

export class PlaceholderimgStack extends cdk.Stack {
    constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const generatorFn = new lambda.Function(this, 'ImageGenerator', {
            runtime: lambda.Runtime.PROVIDED_AL2,
            handler: "dummy",
            code: lambda.Code.fromAsset("./generator/generator.zip")
        })

        const generatorIntegration = new apigw2integrarion.LambdaProxyIntegration({
            handler: generatorFn
        })

        const apiCert = acm.Certificate.fromCertificateArn(this, 'apigwcert', deployCfg.apigwCertificateArn);

        const apiDomainName = new apigw2.DomainName(this, 'DomainName', {
            domainName: deployCfg.apiDomainName,
            certificate: apiCert,
        });

        // TODO: Setup milder throttling when apigw2 cdk is updated.
        const api = new apigw2.HttpApi(this, 'imageGeneratorAPI', {
            defaultDomainMapping:{
                domainName: apiDomainName,
            }
        })
        // TODO: Remove ts-ignore when ts types are updated
        // @ts-ignore
        api.addRoutes({
            path: '/generate',
            methods: [ apigw2.HttpMethod.GET ],
            integration: generatorIntegration
        })

        const webStaticS3 = new s3.Bucket(this, 'PlaceholderimgWeb', {
            versioned: false,
            removalPolicy: RemovalPolicy.DESTROY,
            autoDeleteObjects: true,
            bucketName: deployCfg.frontendS3BucketName,
            publicReadAccess: true
        })

        new s3deploy.BucketDeployment(this, 'DeployWebsite', {
            sources: [s3deploy.Source.asset('./frontend')],
            destinationBucket: webStaticS3,
        })

        const cdnCert = acm.Certificate.fromCertificateArn(this, 'cdncert', deployCfg.cloudFrontCertificateArn);
        new cloudfront.CloudFrontWebDistribution(this, 'PlaceholderimgCDN', {
            originConfigs: [
                {
                    behaviors: [{
                        isDefaultBehavior: true,
                    }],
                    s3OriginSource: {
                        s3BucketSource: webStaticS3
                    },
                }
            ],
            defaultRootObject: "index.html",
            priceClass: PriceClass.PRICE_CLASS_100,
            viewerCertificate: cloudfront.ViewerCertificate.fromAcmCertificate(cdnCert, {
                aliases: deployCfg.wwwDomainNames
            })
        });

    }
}
