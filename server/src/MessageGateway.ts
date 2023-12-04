import { Logger } from '@nestjs/common';
import { OnEvent } from '@nestjs/event-emitter';
import {
  OnGatewayConnection,
  OnGatewayDisconnect,
  OnGatewayInit,
  SubscribeMessage,
  WebSocketGateway,
  WebSocketServer,
} from '@nestjs/websockets';
import { Server, Socket } from 'socket.io';
import { AnimationDelivered } from './app.controller';
import { log } from 'console';

@WebSocketGateway(81, { transports: ['websocket'], cors: true })
export class MessageGateway
  implements OnGatewayInit, OnGatewayConnection, OnGatewayDisconnect
{
  [x: string]: any;
  private logger: Logger = new Logger('MessageGateway');

  @WebSocketServer() wss: Server;

  afterInit(server: Server) {
    this.logger.log('Initialized');
  }

  handleDisconnect(client: Socket) {
    this.logger.log(`Client Disconnected: ${client.id}`);
  }

  handleConnection(client: Socket, ...args: any[]) {
    this.logger.log(`Client Connected: ${client.id}`);
  }

  @SubscribeMessage('sendMessage')
  async handleSendMessage(client: Socket, payload: string): Promise<void> {
    console.log('recived');
    this.wss.emit('receiveMessage', 'aaa');
  }

  @OnEvent('animation_delivered')
  handleOrderCreatedEvent(payload: AnimationDelivered) {
    // handle and process "OrderCreatedEvent" event
    console.log('websocket emit');
    console.log(payload);
    this.wss.emit('reciveAnimation', {
      animation: payload.animationData,
      speed: payload.speed,
    });
  }
}
