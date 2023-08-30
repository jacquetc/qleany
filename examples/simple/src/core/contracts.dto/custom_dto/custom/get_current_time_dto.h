// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>




namespace Simple::Contracts::DTO::Custom
{

class GetCurrentTimeDTO
{
    Q_GADGET


  public:
    GetCurrentTimeDTO() : 
    {
    }

    ~GetCurrentTimeDTO()
    {
    }

    GetCurrentTimeDTO() 
        : 
    {
    }

    GetCurrentTimeDTO(const GetCurrentTimeDTO &other) : 
    {
    }

    GetCurrentTimeDTO &operator=(const GetCurrentTimeDTO &other)
    {
        if (this != &other)
        {
            
        }
        return *this;
    }

    friend bool operator==(const GetCurrentTimeDTO &lhs, const GetCurrentTimeDTO &rhs);


    friend uint qHash(const GetCurrentTimeDTO &dto, uint seed) noexcept;




  private:

};

inline bool operator==(const GetCurrentTimeDTO &lhs, const GetCurrentTimeDTO &rhs)
{

    return 
            
    ;
}

inline uint qHash(const GetCurrentTimeDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        

        return hash;
}

} // namespace Simple::Contracts::DTO::Custom
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Custom::GetCurrentTimeDTO)