# censor-ru-service

## Add file config Blocky for blacklist
###### config `/etc/blocky/config.yml`
```
# ... Start fragment

blocking:
  # Возвращать 0.0.0.0 для заблокированных IPv4 запросов
  blockType: zeroIp
  
  blackLists:
    ru_blocklist:
      - ./blocklist-ru-domain.txt
  
  clientGroupsBlock:
    default:
      - ru_blocklist

# ... End fragment
```
