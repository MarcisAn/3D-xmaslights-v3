import { Body, Controller, Get, Post, Req, Request } from '@nestjs/common';
import { AppService } from './app.service';
import { DBService } from './db.service';
import { firstValueFrom } from 'rxjs';
import { EventEmitter2 } from '@nestjs/event-emitter';
import {readFileSync} from "fs";

export class NewCords {
  cords: string;
  index: string;
}

@Controller()
export class AppController {
  constructor(
    private readonly appService: AppService,
    private eventEmitter: EventEmitter2,
    private dbService: DBService,
  ) {}

  @Post('requestanim')
  async requestAnim(
    @Req()
    request: Request,
  ): Promise<string> {
    const animation = this.appService.getAnim(request.body);
    this.eventEmitter.emit(
      'animation_delivered',
      //@ts-ignore
      new AnimationDelivered(await animation, request.body.speed),
    );
    return '';
  }

  @Get('getAnimCache')
  async getAnimCache(
      @Req()
      request: Request
  ): Promise<string>{
    const cache = readFileSync('./cache.json');
    return cache.toString();
  }

  @Post('updateCords')
  updateCords(@Body() cords: NewCords) {
    this.appService.updateCords(cords.cords, cords.index);
  }

  @Get('getAnims')
  getAnims() {
    return this.dbService.getAllAnims();
  }

  /*
  @Get('chords')
  getChords(
    @Req()
    request: Request,
  ): string {
    //console.log(request);

    return 'aa';
  }*/
}

export class AnimationDelivered {
  animationData: string;
  speed: string;

  constructor(animationData: string, speed: number) {
    this.animationData = animationData;
    this.speed = speed.toString();
  }
}
