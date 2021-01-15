// You must change these values to deploy your copy of the website.
import * as dotenv from "dotenv";
dotenv.config();
let path;
switch (process.env.NODE_ENV) {
    case "test":
        path = `${__dirname}/../.env.test`;
        break;
    case "production":
        path = `${__dirname}/../.env.prod`;
        break;
    default:
        path = `${__dirname}/../.env.dev`;
}
dotenv.config({ path: path });

export const deployCfg = {
    frontendS3BucketName: process.env.FRONTEND_S3_BUCKET_NAME || "uniquebucketname",
    apiDomainName: process.env.API_DOMAIN_NAME || "api.placeholderimg.net",
    apigwCertificateArn: process.env.CERTIFICATE_ARN || "arn:aws:acm:XXXX:XXXX:certificate/XXXX",
    wwwDomainNames: (process.env.WWW_DOMAIN_NAME || "www.placeholderimg.net").split(","),
    cloudFrontCertificateArn: process.env.CLOUDFRONT_CERTIFICATE_ARN || "arn:aws:acm:us-east1:XXXX:certificate/XXXX",
}
