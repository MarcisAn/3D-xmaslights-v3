import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { FetchModule } from 'nestjs-fetch';
import { HttpModule } from '@nestjs/axios';
import { MessageGateway } from './MessageGateway';
import { EventEmitterModule } from '@nestjs/event-emitter';
import { ConfigModule } from '@nestjs/config';
import { PrismaService } from './prisma.service';
import { DBService } from './db.service';

@Module({
  imports: [
    FetchModule,
    HttpModule,
    EventEmitterModule.forRoot(),
    ConfigModule.forRoot(),
  ],
  controllers: [AppController],
  providers: [AppService, MessageGateway, PrismaService, DBService],
})
export class AppModule {}
