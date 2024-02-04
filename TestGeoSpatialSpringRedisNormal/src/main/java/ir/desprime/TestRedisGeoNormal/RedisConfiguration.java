package ir.desprime.TestRedisGeoNormal;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.data.redis.core.GeoOperations;
import org.springframework.data.redis.core.RedisTemplate;

@Configuration
public class RedisConfiguration {

    @Bean
    public GeoOperations<String,String> geoOperations(RedisTemplate<String,String> template) {
        return template.opsForGeo();
    }

}
