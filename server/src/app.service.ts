import { HttpService } from '@nestjs/axios';
import { Injectable, Post } from '@nestjs/common';
import { AxiosResponse } from 'axios';
import { FetchService } from 'nestjs-fetch';
import { Observable, firstValueFrom } from 'rxjs';
import { DBService } from './db.service';

@Injectable()
export class AppService {
  constructor(
    private readonly httpService: HttpService,
    private db: DBService,
  ) {}

  async getAnim(body: ReadableStream<Uint8Array>): Promise<string> {
    console.log(body);
    const url = 'http://127.0.0.1:8000';
    //console.log(await this.db.getAllCords());

    const { data } = await firstValueFrom(this.httpService.post(url, body));
    //console.log(data);
    return data;
  }

  updateCords(cords: string, index) {
    const split = cords.split(',');
    this.db.updateCords(
      { index: Number(index + 1) },
      { x: Number(split[0]), y: Number(split[1]), z: Number(split[2]) },
    );
  }
}
