#pragma once

#include "QtCore/qobject.h"
#include "qleany/qleany_export.h"
#include <QList>
#include <QString>

#define QLN_ERROR_1(func_info, err_type, error_code) Error(func_info, err_type, error_code, __FILE__, __LINE__)
#define QLN_ERROR_2(func_info, err_type, error_code, msg)                                                              \
    Error(func_info, err_type, error_code, msg, __FILE__, __LINE__)
#define QLN_ERROR_3(func_info, err_type, error_code, msg, data)                                                        \
    Error(func_info, err_type, error_code, msg, data, __FILE__, __LINE__)

namespace Qleany
{
/**
 * @brief The Error class represents an error with an associated status, error code, and optional error message and
 * data.
 */
class QLEANY_EXPORT Error
{
    Q_GADGET
    Q_PROPERTY(Status status READ status)
    Q_PROPERTY(QString code READ code)
    Q_PROPERTY(QString message READ message)
    Q_PROPERTY(QString data READ data)
    Q_PROPERTY(QString className READ className)

  public:
    /**
     * @brief The Status enum defines the possible error statuses.
     */
    enum Status
    {
        Ok,       ///< The operation succeeded without any issues.
        Warning,  ///< The operation succeeded but with some non-fatal issues or warnings.
        Critical, ///< The operation failed with a critical error.
        Fatal,    ///< The operation failed with a fatal error.
        Empty     ///< The error is empty or uninitialized.
    };
    Q_ENUM(Status)

    /**
     * @brief Constructs an Error object with the given QObject, status, and error code.
     *
     * @param object The QObject associated with the error.
     * @param status The error status.
     * @param code The error code.
     */
    explicit Error(const QObject *object, const Error::Status &status, const QString &code, const char *file, int line)
        : m_status(status), m_code(code), m_message(""), m_data(""), m_file(file), m_line(line)
    {
        m_className = object->metaObject()->className();
    }

    /**
     * @brief Constructs an Error object with the given QObject, status, error code, and error message.
     *
     * @param object The QObject associated with the error.
     * @param status The error status.
     * @param code The error code.
     * @param message The error message.
     */
    explicit Error(const QObject *object, const Error::Status &status, const QString &code, const QString &message,
                   const char *file, int line)
        : m_status(status), m_code(code), m_message(message), m_data(""), m_file(file), m_line(line)
    {
        m_className = object->metaObject()->className();
    }

    //--------------------------------------------------------------
    explicit Error(const QObject *object, const Error::Status &status, const QString &code, const QString &message,
                   const QString data, const char *file, int line)
        : m_status(status), m_code(code), m_message(message), m_data(data), m_file(file), m_line(line)
    {
        m_className = object->metaObject()->className();
    }

    //--------------------------------------------------------------
    explicit Error(const QString &className, const Error::Status &status, const QString &code, const char *file,
                   int line)
        : m_className(className), m_status(status), m_code(code), m_message(""), m_data(""), m_file(file), m_line(line)
    {
    }

    //--------------------------------------------------------------
    explicit Error(const QString &className, const Error::Status &status, const QString &code, const QString &message,
                   const char *file, int line)
        : m_className(className), m_status(status), m_code(code), m_message(message), m_data(""), m_file(file),
          m_line(line)
    {
    }

    //--------------------------------------------------------------
    explicit Error(const QString &className, const Error::Status &status, const QString &code, const QString &message,
                   const QString data, const char *file, int line)
        : m_className(className), m_status(status), m_code(code), m_message(message), m_data(data), m_file(file),
          m_line(line)
    {
    }

    /**
     *         @brief Constructs an empty Error object.
     *   Initializes the Error with an empty error status.
     */
    explicit Error()
    {
        m_status = Status::Empty;
    }

    //--------------------------------------------------------------
    Error(const Error &other)
        : m_status(other.m_status), m_className(other.m_className), m_code(other.m_code), m_message(other.m_message),
          m_data(other.m_data), m_file(other.m_file), m_line(other.m_line), m_trace(other.m_trace)
    {
    }

    bool operator==(const Error &otherError) const
    {
        return m_status == otherError.m_status && m_className == otherError.m_className &&
               m_code == otherError.m_code && m_message == otherError.m_message;
    }

    bool operator!=(const Error &otherError) const
    {
        return m_status != otherError.m_status || m_className != otherError.m_className ||
               m_code != otherError.m_code || m_message != otherError.m_message;
    }

    /**
     * @brief Returns the error message.
     *
     * @return The error message.
     */
    Q_INVOKABLE QString message() const
    {
        return m_message;
    }
    /**
     * @brief Returns the error data.
     *
     * @return The error data.
     */
    Q_INVOKABLE QString data() const
    {
        return m_data;
    }

    QString stackTrace() const
    {
        return m_file + ":" + QString::number(m_line);
    }

    /**
     * @brief Returns true if the error status is Ok, false otherwise.
     *
     * @return True if the error status is Ok, false otherwise.
     */
    bool isOk() const
    {
        return m_status == Status::Ok;
    }

    /**
     * @brief Returns true if the error status is Empty, false otherwise.
     *
     * @return True if the error status is Empty, false otherwise.
     */
    bool isEmpty() const
    {
        return m_status == Status::Empty;
    }

    /**
     * @brief Returns the error status.
     *
     * @return The error status.
     */
    Error::Status status() const
    {
        return m_status;
    }

    /**
     * @brief Sets the error status.
     *
     * @param status The new error status.
     */
    void setStatus(const Error::Status &status)
    {
        m_status = status;
    }

    //--------------------------------------------------------------

    Q_INVOKABLE QString code() const
    {
        return m_code;
    }

    Q_INVOKABLE QString className() const;

    QList<Error> trace() const
    {
        return m_trace;
    }
    void setTrace(const QList<Error> &newTrace)
    {
        m_trace = newTrace;
    }

  private:
    QString m_code;
    QString m_message;
    QString m_data;
    QString m_className;
    Error::Status m_status;
    QString m_file;
    int m_line;
    QList<Error> m_trace;
};

inline QString Error::className() const
{
    return m_className;
}

}; // namespace Qleany
Q_DECLARE_METATYPE(Qleany::Error)
