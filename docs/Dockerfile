FROM blang/latex:ctanbasic

RUN apt-get update
RUN apt-get install -y python-pygments

RUN tlmgr option repository ftp://tug.org/historic/systems/texlive/2017/tlnet-final
RUN tlmgr update --self
RUN tlmgr install xcolor
RUN tlmgr install minted
RUN tlmgr install mdframed
RUN tlmgr install texliveonfly
RUN tlmgr install fancyvrb
RUN tlmgr install etoolbox
RUN tlmgr install fvextra
RUN tlmgr install pgf

VOLUME /data
WORKDIR /data
COPY . .

RUN texliveonfly final/index.tex; exit 0
