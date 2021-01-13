#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { PlaceholderimgStack } from '../lib/placeholderimg-stack';

const app = new cdk.App();
new PlaceholderimgStack(app, 'PlaceholderimgStack');
