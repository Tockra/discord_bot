# discord_bot
Dieser Discord-Bot wird verwendet um mittels Chatkommando einen docker-container zu starten und zu stoppen. Dieser Bot wird selbst in einem Docker-Container mittels `docker-compose`ausgeführt.

Hierbei wird in den Dockercontainer eine Docker-Instanz installiert und die externe Docker.sock in diesen Container gemountet. Somit kann aus dem Container heraus der andere Container gesteuert werden. In diesem Fall ist es der minecraft-container, dessen docker-compose-file auch in den Container gemountet wird. Für andere Anwendungen muss dieser Mount angepasst werden.

In dem docker-compose-file muss die Environment-Variable `TOKEN` angepasst werden, damit der Bot zum richtigen Server connectet.
