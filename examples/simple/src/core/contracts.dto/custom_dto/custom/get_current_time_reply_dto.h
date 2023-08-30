// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>
#include <QDateTime>




namespace Simple::Contracts::DTO::Custom
{

class GetCurrentTimeReplyDTO
{
    Q_GADGET

    Q_PROPERTY(QDateTime currentDateTime READ currentDateTime WRITE setCurrentDateTime)

  public:
    GetCurrentTimeReplyDTO() : m_currentDateTime(QDateTime())
    {
    }

    ~GetCurrentTimeReplyDTO()
    {
    }

    GetCurrentTimeReplyDTO( const QDateTime &currentDateTime ) 
        : m_currentDateTime(currentDateTime)
    {
    }

    GetCurrentTimeReplyDTO(const GetCurrentTimeReplyDTO &other) : m_currentDateTime(other.m_currentDateTime)
    {
    }

    GetCurrentTimeReplyDTO &operator=(const GetCurrentTimeReplyDTO &other)
    {
        if (this != &other)
        {
            m_currentDateTime = other.m_currentDateTime;
            
        }
        return *this;
    }

    friend bool operator==(const GetCurrentTimeReplyDTO &lhs, const GetCurrentTimeReplyDTO &rhs);


    friend uint qHash(const GetCurrentTimeReplyDTO &dto, uint seed) noexcept;



    // ------ currentDateTime : -----

    QDateTime currentDateTime() const
    {
        return m_currentDateTime;
    }

    void setCurrentDateTime( const QDateTime &currentDateTime)
    {
        m_currentDateTime = currentDateTime;
    }
    


  private:

    QDateTime m_currentDateTime;
};

inline bool operator==(const GetCurrentTimeReplyDTO &lhs, const GetCurrentTimeReplyDTO &rhs)
{

    return 
            lhs.m_currentDateTime == rhs.m_currentDateTime 
    ;
}

inline uint qHash(const GetCurrentTimeReplyDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_currentDateTime, seed);
        

        return hash;
}

} // namespace Simple::Contracts::DTO::Custom
Q_DECLARE_METATYPE(Simple::Contracts::DTO::Custom::GetCurrentTimeReplyDTO)