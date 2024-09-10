# 설명

1. 프로그램 파일

   - 리눅스 : file_download_api

   - 윈도우 : file_download_api.exe

2. 기본 포트 : 9991 (프로그램 실행시 포트 지정 가능)
   
3. 리눅스 실행 방법

   3.1 다운받을 파일이 있는 서버에 file_download_api 파일 업로드

   3.2 실행 명령어

       - 기본 : nohup ./file_download_api &

	   - 포트 지정 : PORT={포트입력} nohup ./file_download_api &

                ex. PORT=8010 nohup ./file_download_api &


# api 명세서


***
### 📬 **GET**

- Endpoint : **[GET]**  /fiie?path=`{다운로드 file path}`

- Description: 단일 파일 다운로드

- Request  Example:

```
 http://13.209.120.222:9991/file?path=/home/ec2-user/fileDownload/nohup.out
```


- Response Header Example:

```
{
    "content_type": "application/octet-stream",
    "content-disposition": attachment; filename="nohup.out"
}
```
- ERROR `404` : 
    - 없는 파일을 요청한 경우 : File not found
    - 파일은 존재하나, 오픈이 불가능한 경우 : Could not open file
    - 파일은 존재하나, 읽는 게 불가능한 경우 : Could not read file
***

***
### 📬 **GET**

- Endpoint : **[GET]**  /fiies?path=`{다운로드 file path1}, {다운로드 file path2}`

- Description: 어러개의 파일 다운로드 (다운로드 되는 파일은 1번째 path의 확장자 포함된 이름 + .zip)

     ex. /home/ec2-user/fileDownload/nohup.out
         /home/ec2-user/inzentweb/xedm-project-0.0.1-SNAPSHOT.jar
         두 개 파일 요청 시, nohup.out.zip으로 압축되어 다운로드 됨

- Request  Example:

```
 http://13.209.120.222:9991/files?path=/home/ec2-user/fileDownload/nohup.out,/home/ec2-user/inzentweb/xedm-project-0.0.1-SNAPSHOT.jar
```


- Response Header Example:

```
{
    "content_type": "application/zip",
    "content-disposition": attachment; filename="nohup.out.zip"
}
```
- ERROR `404` : 
    - 없는 파일을 요청한 경우 : File not found
    - 파일은 존재하나, 오픈이 불가능한 경우 : Could not open file
    - 파일은 존재하나, 읽는 게 불가능한 경우 : Could not read file
***


***
### 📬 **GET**

- Endpoint : **[GET]**  /folder?path=`{다운로드 folder path1}`

- Description: 단일 폴더 다운로드 ( 다운로드 파일 이름은 폴더 이름 + .zip)

     ex. /home/ec2-user/fileDownload/ 해당 폴더 하위의 모든 파일을 다운로드 요청, fileDownload.zip으로 다운로드 됨.

- Request  Example:

```
 http://13.209.120.222:9991/folder?path=/home/ec2-user/fileDownload/
```


- Response Header Example:

```
{
    "content_type": "application/zip",
    "content-disposition": attachment; filename="fileDownload.zip"
}
```
- ERROR `404` : 
    - 없는 폴더를 요청한 경우 : Folder not found
    - 파일은 존재하나, 오픈이 불가능한 경우 : Could not open file
    - 파일은 존재하나, 읽는 게 불가능한 경우 : Could not read file
***