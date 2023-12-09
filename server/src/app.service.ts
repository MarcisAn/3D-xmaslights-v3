import { HttpService } from '@nestjs/axios';
import { Injectable, Post } from '@nestjs/common';
import { AxiosResponse } from 'axios';
import { FetchService } from 'nestjs-fetch';
import { Observable, firstValueFrom } from 'rxjs';
import { DBService } from './db.service';
import { readFileSync, writeFile } from 'fs';

@Injectable()
export class AppService {
  constructor(
    private readonly httpService: HttpService,
    private db: DBService,
  ) {}

  async getAnim(body: ReadableStream<Uint8Array>): Promise<string> {
    console.log(body);
    const url = 'http://127.0.0.1:8000';
    // ja ir kešota animācija
    if (this.getCacheIfExists(body) != null) {
      return this.getCacheIfExists(body).data;
    }
    const { data } = await firstValueFrom(this.httpService.post(url, body));

    this.createCache(body, data)
    return data;
  }

  updateCords(cords: string, index) {
    const split = cords.split(',');
    this.db.updateCords(
      { index: Number(index + 1) },
      { x: Number(split[0]), y: Number(split[1]), z: Number(split[2]) },
    );
  }

  getCacheIfExists(req){
    const cacheIndex = readFileSync('./cache.json');
    //@ts-ignore
    const cachedAnims = JSON.parse(cacheIndex);
    let index_in_cache = 0;
    for (const anim of cachedAnims){
      if (req.name == anim.name && JSON.stringify(req.colors) == JSON.stringify(anim.colors)){
        return anim;
      }
      index_in_cache++;
    }
    return null;
  }

  createCache(req, data){
    const cacheIndex = readFileSync('./cache.json');
    //@ts-ignore
    let cachedAnims = JSON.parse(cacheIndex);
    req.data = data;
    //console.log(req)
    cachedAnims.push(req);
    writeFile("cache.json", JSON.stringify(cachedAnims), () => {});
  }

}
